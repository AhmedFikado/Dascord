import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from 'lucide-react';
import { useState } from "react";


export default function CreateServerDialog() {

    const [isDialogOpen, setIsDialogOpen] = useState(false);

    return (
        <>
            <Button
                width="45px"
                height="45px"
                style={{ borderRadius: '1rem', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsDialogOpen(true)}>
                <Plus />
            </Button>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Créer un serveur"
            >
                <form action="submit" className="flex flex-col gap-6">
                    <Input
                        label="Nom du serveur"
                        type="text"
                        placeholder="Nom du serveur"
                    />

                    <Button variant={'primary'} type="submit">Créer le serveur</Button>

                </form>
            </Dialog>
        </>

    )
}