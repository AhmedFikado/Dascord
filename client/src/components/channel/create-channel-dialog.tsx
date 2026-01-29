import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from 'lucide-react';
import { useState } from "react";

export default function CreateChannelDialog() {
    const [isDialogOpen, setIsDialogOpen] = useState(false);

    return (
        <>
            <Button
                width="26px"
                height="26px"
                style={{ borderRadius: '10px', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsDialogOpen(true)}>
                <Plus size={14} />
            </Button>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Créer un channel"
            >
                <form action="submit" className="flex flex-col gap-8">
                    <Input
                        label="Nom du channel"
                        type="text"
                        placeholder="Nom du channel"
                    />

                    <Button variant={'primary'} width="250px" style={{ alignSelf: 'center' }} type="submit">Créer le channel</Button>

                </form>
            </Dialog>
        </>
    )
}
