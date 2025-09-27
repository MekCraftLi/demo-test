use tauri::command;

use chrono::prelude::*;
use serde::Serialize;
use serialport::available_ports;
use std::thread;
use std::time::Duration;
use tauri::{ipc::Channel, AppHandle, Manager};
use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialPacket {
    million_sec: u32,
    index: usize,
    data: Vec<u8>,
}

pub struct TaskState {
    running: Arc<AtomicBool>,
}

#[command]
pub fn start_serial_simulation(
    app: AppHandle,
    on_event: Channel<SerialPacket>,
    window_label: String,
) {
    thread::spawn(move || {
        let mut packet_id = 0;

        loop {
            if packet_id == 10000 {
                break;
            }
            packet_id += 1;
            let now: DateTime<Local> = Local::now();

            // 提取时、分、秒、毫秒
            let hour = now.hour();
            let minute = now.minute();
            let second = now.second();
            let millisecond = now.timestamp_subsec_millis();
            let millis_since_midnight =
                hour * 3600_000 + minute * 60_000 + second * 1000 + millisecond;

            let data = vec![packet_id as u8; 128];
            let packet = SerialPacket {
                million_sec: millis_since_midnight,
                index: packet_id,
                data,
            };

            // 检查窗口是否存在
            if let Some(_win) = app.get_webview_window(&window_label) {
                // 窗口存在 → 发送数据
                if let Err(e) = on_event.send(packet.clone()) {
                    eprintln!("发送失败: {:?}", e);
                }
            } else {
                // 窗口不存在 → 不发送
                // 可选：打印日志或直接忽略
                // eprintln!("窗口 {} 不存在，跳过发送", window_label);
            }

            // 1 kHz
            thread::sleep(Duration::from_millis(100));
        }
    });
}

#[command]
pub async fn start_receive_comm(
    app: AppHandle,
    on_event: Channel<SerialPacket>,
    label: String,
    state: tauri::State<'_, Mutex<Option<TaskState>>>,
) -> Result<(), String> {

    {
        let mut guard = state.lock().await;

        // 检查是否已有任务
        if let Some(task) = &*guard {
            if task.running.load(Ordering::SeqCst) {
                println!("窗口已被创建");
                return Ok(());
            }
        }
    }

    let ports = available_ports().expect("无法获取串口");
    let packet_id = 0;


    // 遍历每个串口设备
    for port in ports {
        if let serialport::SerialPortType::UsbPort(info) = port.port_type {
            if let Some(manufacturer) = info.manufacturer {
                if manufacturer == "STMicroelectronics.".to_string()
                    || manufacturer == "MekLi".to_string()
                {
                    let mut port = tokio_serial::new(port.port_name, 1152000)
                        .open_native_async()
                        .expect("串口开启失败");

                    let mut buf = vec![0; 1024];

                    /* 任务控制 */
                    let running = Arc::new(AtomicBool::new(true));
                    let r = running.clone();

                    let on_event = on_event.clone();

                    {
                        /* 拿到全局变量，赋值为running = true */
                        let mut guard = state.lock().await;
                        *guard = Some(TaskState{running});
                    }

                    let async_state = state.clone();

                    tokio::spawn(async move {

                        /* 后台发送线程 */

                        while r.load(Ordering::SeqCst) {
                            match port.read(&mut buf).await {
                                Ok(n) => {
                                    println!("收到{}字节数据:{:?}", n, &buf[..n]);

                                    let now: DateTime<Local> = Local::now();

                                    // 提取时、分、秒、毫秒
                                    let hour = now.hour();
                                    let minute = now.minute();
                                    let second = now.second();
                                    let millisecond = now.timestamp_subsec_millis();
                                    let millis_since_midnight =
                                        hour * 3600_000 + minute * 60_000 + second * 1000 + millisecond;

                                    let mut data = Vec::new();
                                    data.extend_from_slice(&buf[..n]);
                                    let packet = SerialPacket {
                                        million_sec: millis_since_midnight,
                                        index: packet_id,
                                        data,
                                    };

                                    println!("通道ID:{:?}", on_event.id());

                                    if let Err(e) = on_event.send(packet.clone()) {
                                        eprintln!("发送失败: {:?}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("读取串口失败,{}", e);
                                }
                            }
                        }

                        println!("接收线程停止");

                    });


                }
            }
        }
    }

    println!("开启串口调用结束");

    Ok(())
}

#[command]
pub async fn stop_receive_comm(
    app: AppHandle,
    state: tauri::State<'_, Mutex<Option<TaskState>>>,
) -> Result<(), String> {

    let mut guard = state.lock().await;

    if let Some(task) = guard.take() {
        task.running.store(false, Ordering::SeqCst);
        println!("停止后台任务")
    } else {
        println!("无运行任务")
    }



    Ok(())
}
