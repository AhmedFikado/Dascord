import { Channel } from "@/types/models/channel";
import ChannelTrash from "./channel-trash";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { Input } from "../ui/input";
import { Dialog } from "../ui/dialog";

export default function ChannelItem({ channel }: { channel: Channel }) {

    const [isDialogOpen, setIsDialogOpen] = useState(false);

    const deleteChannel = async () => {
        console.log("Deleting channel");
        setIsDialogOpen(false);
    }

    return (
        <>
            <li key={channel.id} className='group pl-3 pr-3 py-[6px] rounded-lg cursor-pointer hover:bg-hoverSide flex justify-between items-center'>
                <div className="flex items-center text-gray-light group-hover:text-white text-left">
                    {channel.name}
                </div>
                <div>
                    <Button onClick={() => setIsDialogOpen(true)} variant="noBackground" width="26px" height="26px" style={{ borderRadius: '10px', padding: 0 }}>
                        <ChannelTrash></ChannelTrash>
                    </Button>
                </div>
            </li >

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Supprimer un channel"
            >
                <h2 className="text-white mb-8 text-center">Êtes-vous sûr de vouloir supprimer ce channel ?</h2>

                <div className="flex gap-4 justify-center">
                    <Button onClick={() => deleteChannel()} variant={'danger'} width="200px" style={{ alignSelf: 'center' }} type="submit">Supprimer le channel</Button>
                    <Button onClick={() => setIsDialogOpen(false)} variant={'secondary'} width="100px" style={{ alignSelf: 'center' }} type="submit">Annuler</Button>
                </div>

            </Dialog>

        </>
    )

}