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
  parent_id: string
  path: string
  localImages: string[]
  isMerged: boolean
  metadata: ModMetadata
}
