import { invoke } from "@tauri-apps/api/tauri";

export type LetterStatus = 'correct' | 'mistake' | 'suggestion'

export interface Letter {
  character: string,
  status: LetterStatus,
}

export async function getCursor(): Promise<number> {
  return invoke('get_cursor')
}

export async function getInput(): Promise<Letter[][]> {
  return invoke('get_input')
}

export async function insert(character: string): Promise<Letter[][]> {
  return invoke('insert', { character: character })
}

export async function backspace(): Promise<Letter[][]> {
  return invoke('backspace')
}
