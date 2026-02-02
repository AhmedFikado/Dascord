

export default function ServerHeader({ channelName }: { channelName: string }) {
    return (
        <header className="h-[61px] bg-background flex items-center px-4 border-b border-gray-200 flex-shrink-0 w-full">
            <h1 className="text-white text-lg font-bold">#{channelName}</h1>
        </header>
    );
}