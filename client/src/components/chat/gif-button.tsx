import { Button } from "../ui/button";
import GifPanel from "./gif-panel"
import { useState } from "react";

interface GifButtonProps {
    channelId: string;
}

export default function GifButton({ channelId }: GifButtonProps) {

    const [isPanelOpen, setIsPanelOpen] = useState(false);

    const togglePanel = () => {
        setIsPanelOpen(!isPanelOpen);
    };

    return (
        <>
            <Button onClick={togglePanel} variant="nothing" width="30px" height="30px" className="text-white hover:bg-gray-100 bg-gray-100/50">
                <p className="text-xs">GIF</p>
            </Button>

            {isPanelOpen && (
                <>
                    <div
                        className="fixed inset-0 z-40"
                        onClick={togglePanel}
                    />
                    <div className="absolute bottom-16 right-0 z-50 w-full sm:w-96 md:w-[400px] lg:w-[450px] max-w-[calc(100vw-2rem)]">
                        <GifPanel channelId={channelId} />
                    </div>
                </>
            )}
        </>
    );
}
