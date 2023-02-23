import { ModType, enums } from './constant'

export function getCategories(str: string): string[] {
  for (let i = 0; i < enums.length; i++) {
    const category = enums[i]
    // @ts-ignore
    const value = category[str]
    if (typeof value === 'string') {
      return [ModType[i], value]
    }
  }
  return [ModType[99]]
}
