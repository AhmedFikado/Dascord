import { Input } from "@/components/ui/input";
import { useState } from "react";
import { GifAPI } from "@/app/lib/api/giphy";
import { useWebSocketContext } from "../shared/websocket-provider";

interface GifButtonProps {
    channelId: string;
}

export default function GifPanel({ channelId }: GifButtonProps) {

    const [search, setSearch] = useState('');
    const { sendChannelMessage } = useWebSocketContext();
    const [results, setResults] = useState<any[]>([]);


    const handleGifChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(e.target.value);
        searchGif();
    };

    const searchGif = async () => {
        GifAPI.getGif(search)
            .then(response => {
                setResults(response.data);
            })
            .catch(error => {
                console.error('Error fetching GIFs:', error);
            });
    };

    const applyGif = async (url: string) => {
        console.log(channelId)
        sendChannelMessage(channelId, url);
    }



    return (
        <div className="w-full bg-gray-400 mt-2 rounded-lg border border-gray-300 shadow-lg">
            <div className="p-3 border-b border-gray-300">
                <Input
                    type="text"
                    required
                    id="gif"
                    name="gif"
                    value={search}
                    onChange={handleGifChange}
                    placeholder="Rechercher un GIF" />
            </div>
            <section className="h-80 overflow-y-auto p-3">
                {results.length > 0 ? (
                    <div className="grid grid-cols-2 sm:grid-cols-3 gap-2 sm:gap-3">
                        {results.map((result) => (
                            <div key={result.id} onClick={() => applyGif(result.images.fixed_height.url)} className="relative aspect-square overflow-hidden rounded-lg hover:opacity-80 transition-opacity cursor-pointer">
                                <img
                                    src={result.images.fixed_height.url}
                                    alt={result.title}
                                    className="w-full h-full object-cover"
                                />
                            </div>
                        ))}
                    </div>
                ) : (
                    <div className="flex items-center justify-center h-full">
                        <p className="text-white text-sm">Aucun résultat trouvé</p>
                    </div>
                )}
            </section>
        </div>
    );
}