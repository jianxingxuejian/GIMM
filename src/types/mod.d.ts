interface ModMetadata {
  info: {
    name: string
    description: string
    urls: string[]
    images: string[]
    videos: string[]
  }
  author: {
    name: string
    urls: string[]
    sponsor: string[]
  }
  categories: string[]
  tags: string[]
  order: number
  like: boolean
}

interface ModInfo extends ModMetadata {
  id: string
  parentId: string
  path: string
  localImages: string[]
}

interface ModParse extends ModInfo {
  iniName: string
  isMerged: boolean
}
