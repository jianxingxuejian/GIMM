import type { OpenDialogOptions } from '@tauri-apps/api/dialog'
import { shell, dialog } from '@tauri-apps/api'
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
import { getCategories } from '@/views/mod/handle'

export function select_dir<T extends OpenDialogOptions = OpenDialogOptions>(options?: T) {
  const { directory = true, multiple = false } = options || {}
  return dialog.open({ directory, multiple }) as Promise<
    T['multiple'] extends true ? null | string[] : null | string
  >
}

export const select_file = <T extends OpenDialogOptions = OpenDialogOptions>(options?: T) =>
  dialog.open(options) as Promise<T['multiple'] extends true ? null | string[] : null | string>

export const open_dir = (path: string) => shell.open(path)

export async function get_mod_list(path?: string): Promise<ModInfo[]> {
  if (!path) return []
  const modInfoList = await invoke<ModParse[]>('get_mod_list', { path })
  return modInfoList.map(({ isMerged, iniName, path, categories, ...other }) => ({
    ...other,
    categories: !isMerged && categories.length === 0 ? getCategories(iniName) : categories,
    path: path.replace(/\\/g, '/'),
  }))
}

export const rename = (path: string, newPath: string) => invoke('rename', { path, newPath })

export function write_file(mod: ModMetadata) {
  // const { path, id, name, images, submitter, type, modId } = mod
  // const contents = JSON.stringify({ id, name, images, submitter, type, modId })
  // return invoke('write_file', { path: path + '/modinfo.json', contents })
}

export function download(url: string, path: string, mod: ModMetadata) {
  // const { id, name, images, submitter } = mod
  // const contents = JSON.stringify({ id, name, images, submitter })
  // return invoke('download', { url, path, contents })
}

export const installCA = () => invoke<string>('install_ca')

export const setProxyAddr = (addr: string) => invoke('set_proxy_addr', { addr })

export const proxyStart = (port: string) => invoke<string>('proxy_start', { port: Number(port) })

export const proxyEnd = () => invoke<string>('proxy_end')

export const runProgram = (path: string, args?: string) =>
  invoke<string>('run_program', { path: path.replace(/\\/g, '/'), args })

export const runJar = (path: string) =>
  invoke<string>('run_jar', { path: path.replace(/\\/g, '/') })

export const executeLuac = (path: string, contents: string) =>
  invoke<string>('execute_luac', { path: path.replace(/\\/g, '/'), contents })

export const getEnableState = () => invoke<boolean>('get_enable_state')
