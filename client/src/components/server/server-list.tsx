import { Server } from "@/types/models/Server";
import ServerItem from "./server-item";

export default function ServerList({ servers }: { servers: Server[] }) {

    return (
        <nav className="">
            <ul>
                {servers.map((server) => (
                    <ServerItem server={server} key={server.id} />
                ))}
            </ul>
        </nav>
    )
}