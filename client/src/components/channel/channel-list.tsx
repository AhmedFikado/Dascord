'use client';

import ChannelItem from './channel-item';
import { Channel } from "@/types/models/channel";

export default function ChannelList({ channels }: { channels: Channel[] }) {

    return (
        <nav>
            <ul>
                {channels.map((channel) => (
                    <ChannelItem channel={channel} />
                ))}
            </ul>
        </nav>
    )

}