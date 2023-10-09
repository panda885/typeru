import { invoke } from "@tauri-apps/api/tauri";

export async function getText(): Promise<string> {
  return invoke('get_text')
}

export async function insert(character: string): Promise<string> {
  return invoke('insert', { character: character })
}

export async function backspace(): Promise<string> {
  return invoke('backspace')
}
