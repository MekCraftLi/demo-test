import type { ClassValue } from "clsx"
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

interface Point{x: number, y: number}

/**
 * 生成三点折线圆角路径
 * @param p0 起点 [x, y]
 * @param p1 折角点 [x, y]
 * @param p2 终点 [x, y]
 * @param r  圆角半径
 * @returns SVG path d 字符串
 */
export function createArcPath(p0: Point, p1: Point, p2: Point, r: number): string {
    // 向量
    const v0 = { x: p0.x - p1.x, y: p0.y - p1.y }
    const v1 = { x: p2.x - p1.x, y: p2.y - p1.y }

    // 归一化
    const len = (v: Point) => Math.hypot(v.x, v.y)
    const normalize = (v: Point) => ({ x: v.x / len(v), y: v.y / len(v) })
    const u0 = normalize(v0)
    const u1 = normalize(v1)

    // 夹角
    const cosθ = u0.x * u1.x + u0.y * u1.y
    const θ = Math.acos(cosθ)

    // 切点距离
    const t = r / Math.tan(θ / 2)
    const T0 = { x: p1.x + u0.x * t, y: p1.y + u0.y * t }
    const T1 = { x: p1.x + u1.x * t, y: p1.y + u1.y * t }

    // 计算圆心方向
    const cross = u0.x * u1.y - u0.y * u1.x
    //const sign = cross < 0 ? -1 : 1
    //const angle = θ / 2
    //const sinHalf = Math.sin(angle)
    //const C = { x: T0.x + sign * u0.y * r / sinHalf, y: T0.y - sign * u0.x * r / sinHalf }

    // SVG path
    return `${p0.x},${p0.y} L${T0.x},${T0.y} A${r},${r} 0 0,${cross < 0 ? 1 : 0} ${T1.x},${T1.y} L${p2.x},${p2.y}`
}