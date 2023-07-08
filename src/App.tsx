import { useEffect, useState } from 'react'
import init, {
  fibo,
  fibonacci,
  get_wasm_memory_buffer_pointer,
  read_wasm_memory_buffer_and_return_index_one,
  store_value_in_wasm_memory_buffer_index_zero,
} from 'tcm'
import { Knob } from './components'
import './styles/App.css'

function fib2(n: number): number {
  if (n <= 1) return n

  let prev2 = 0
  let prev1 = 1
  let c = 0

  for (let i = 2; i <= n; i++) {
    c = prev1 + prev2
    prev2 = prev1
    prev1 = c
  }

  return c
}

// function fib3(n: number): number {
//   const sol = [0, 1]

//   for (let i = 2; i <= n; i++) {
//     sol[i] = sol[i - 1] + sol[i - 2]
//   }

//   return sol[n]
// }

function App() {
  const [value, setValue] = useState(0)

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    // initialize()
  }, [])

  // async function initialize() {
  //   const t2 = performance.now()
  //   const f2 = fib2(100000)
  //   const t3 = performance.now()
  //   console.log(`Fibonacci JS01 es: ${f2} y ha tardado ${t3 - t2}ms`)

  //   const wasm = await init()
  //   const t0 = performance.now()
  //   const f = fibonacci(100000)
  //   const t1 = performance.now()
  //   console.log(`Fibonacci WASM es: ${f} y ha tardado ${t1 - t0}ms`)

  //   const t4 = performance.now()
  //   const f3 = fibo(100000)
  //   const t5 = performance.now()
  //   console.log(`Fibonacci WASM es: ${f3} y ha tardado ${t5 - t4}ms`)

  //   /**
  //    * Write in Wasm, Read in JS
  //    */
  //   console.log('Write in Wasm, Read in JS, Index 0:')
  //   // write to our buffer
  //   store_value_in_wasm_memory_buffer_index_zero(24)
  //   // Uint8Array of our wasm memory
  //   const memo = new Uint8Array(wasm.memory.buffer)
  //   // Pointer to our buffer that is within wasmMemory
  //   const pointer = get_wasm_memory_buffer_pointer()
  //   // Read the written value at index zero of the buffer
  //   // by accessing the index of wasmMemory[bufferPointer + bufferIndex]
  //   console.log(memo[pointer + 0])

  //   /**
  //    * Part two: Write in JS, Read in Wasm
  //    */
  //   console.log('Write in JS, Read in Wasm, Index 1:')

  //   // First, let's write to index one of our buffer
  //   memo[pointer + 1] = 15

  //   // Then, let's have wasm read index one of the buffer,
  //   // and return the result
  //   console.log(read_wasm_memory_buffer_and_return_index_one())
  // }

  const handleKnobChange = (value: number) => {
    // console.log(value / 10)
    setValue(value)
  }

  return (
    <>
      <h1>Vite + React + WASM</h1>

      <Knob
        size={60}
        degrees={264}
        min={-10}
        max={10}
        value={value}
        label='Knob'
        onChange={handleKnobChange}
      />
    </>
  )
}

export default App
