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
  id: string
  parentId: string
  metadataPath: string
  iniPath: string
  metadata: ModMetadata
  localImages: string[]
  is_disabled: boolean
  isMerged: boolean
  children: string[]
  deepChildren: ModInfo[]
}
