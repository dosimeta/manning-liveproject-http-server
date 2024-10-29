fn main() {
  for i in 0..10 {
    if i > 7 { println!("Salve WasmEdge!") }
    else if i == 7 { println!("こんにちは  WasmEdge!") }
    else if i == 6 { println!("WasmEdge 你好!") }
    else if i == 5 { println!("guten tag WasmEdge!") }
    else if i == 4 { println!("Bonjour WasmEdge!") }
    else if i == 3 { println!("Hola WasmEdge!") }
    else if i > 0  { println!("Howdy WasmEdge!") }
    else { println!("Hello WasmEdge!") }
  }
}
