import { Server } from "@/types/models/Server";

export default function ServerHeaderSide({ server }: { server: Server }) {
    return (
        <header className="">
            <h1 className="text-white text-lg font-bold">{server.name}</h1>
        </header>
    );
}