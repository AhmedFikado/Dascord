import { apiClient } from './client';

const API_URL = 'api.giphy.com/v1/gifs/search';
const API_KEY = process.env.NEXT_PUBLIC_GIPHY_API_KEY || '';

export const GifAPI = {

    // GET GIF
    getGif: async (query: string) => {
        return await apiClient.get<{ data: any[] }>(`https://${API_URL}?api_key=${API_KEY}&q=${encodeURIComponent(query)}&limit=15`)

    }

}
