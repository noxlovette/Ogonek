export interface UnsplashPhoto {
  id: string;
  urls: PhotoURLs;
  alt_description?: string;
  user: UnsplashUser;
}

export interface PhotoURLs {
  raw: string;
  full: string;
  regular: string;
  small: string;
  thumb: string;
}

export interface UnsplashUser {
  name: string;
  username: string;
}

export interface UnsplashResponse {
  results: UnsplashPhoto[];
  total: number;
  total_pages: number;
}
