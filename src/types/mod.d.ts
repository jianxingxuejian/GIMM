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

interface ModInfo {
  id: number
  parentId: number
  metadataPath: string
  iniPath: string
  metadata: ModMetadata
  localImages: string[]
  isDisabled: boolean
  isMerged: boolean
  children: string[]
  deepChildren: ModInfo[]
}
