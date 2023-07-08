import { CSSProperties, MouseEvent, useRef, useState } from 'react'
import css from './Knob.module.scss'
import { convertRange } from './utils'

interface KnobProps {
  value: number
  degrees?: number
  min?: number
  max?: number
  size?: number
  color?: string
  label?: string
  onChange: (value: number) => void
}

export default function Knob({
  size = 60,
  degrees = 264,
  min = 0,
  max = 100,
  color = '#23F376',
  value,
  label,
  onChange,
}: KnobProps): JSX.Element {
  const startAngle = useRef<number>((360 - degrees) / 2)
  const endAngle = useRef<number>(startAngle.current + degrees)
  const margin = size * 0.15
  const currentY = useRef<number>(0)
  const currentDeg = useRef<number>(
    Math.floor(
      convertRange(min, max, startAngle.current, endAngle.current, value)
    )
  )
  const [rotation, setRotation] = useState<number>(currentDeg.current)

  /**
   * Mouse move
   */
  function onMouseMove(event: globalThis.MouseEvent) {
    if (event.pageY - currentY.current !== 0) {
      currentDeg.current -= event.pageY - currentY.current
    }

    currentY.current = event.pageY

    if (currentDeg.current >= endAngle.current) {
      currentDeg.current = endAngle.current
    } else if (currentDeg.current <= startAngle.current) {
      currentDeg.current = startAngle.current
    }

    const newValue = Math.floor(
      convertRange(
        startAngle.current,
        endAngle.current,
        min,
        max,
        currentDeg.current
      )
    )

    setRotation(currentDeg.current)
    onChange(newValue)
  }

  /**
   * Mouse down
   */
  const onDragStart = (event: MouseEvent<HTMLDivElement>) => {
    event.preventDefault()

    currentY.current = event.pageY

    document.addEventListener('mousemove', onMouseMove, false)
    document.addEventListener('mouseup', () => {
      document.removeEventListener('mousemove', onMouseMove)
    })
  }

  // On-fly style
  const kStyle: CSSProperties = { width: size, height: size }
  const gripStyle = { ...kStyle }
  const outerStyle = { ...kStyle }

  outerStyle.margin = margin
  gripStyle.transform = 'rotate(' + rotation + 'deg)'

  return (
    <div className={css.knob} style={kStyle}>
      <div className={css.outer} onMouseDown={onDragStart}>
        <svg className={css.svg} viewBox='0 0 100 100'>
          <path d='M20,76 A 40 40 0 1 1 80 76' fill='none' stroke='#55595C' />
          <path
            d='M20,76 A 40 40 0 1 1 80 76'
            fill='none'
            stroke={color}
            style={{
              strokeDashoffset:
                // 184 is the stroke-dasharray  
                184 - 184 * ((rotation * 1 - startAngle.current) / degrees),
            }}
          />
        </svg>
        <div className={css.inner} style={gripStyle}>
          <div className={css.grip} style={{ backgroundColor: color }} />
        </div>
      </div>

      {label && <p className={css.label}>{label}</p>}
    </div>
  )
}
