import { Channel } from "@/types/models/channel";
import ChannelSettings from "./channel-settings";

export default function ChannelItem({ channel }: { channel: Channel }) {



    return (
        <li key={channel.id} className='group pl-3 pr-4 py-[6px] rounded-lg cursor-pointer hover:bg-hoverSide flex justify-between items-center'>
            <div className="flex items-center text-gray-light group-hover:text-white text-left">
                {channel.name}
            </div>
            <div>
                <ChannelSettings></ChannelSettings>
            </div>
        </li >


    )

}