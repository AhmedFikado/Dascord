"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Loading as LoadingSpinner } from "@/components/shared/loading-spinner";
import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Dropdown, Option } from "@/components/ui/dropdown";
import { Role } from "@/types/models/role";
import { useSnackbar } from "@/components/shared/error-message";
import ChannelList from "@/components/channel/channel-list";
import { Channel } from "@/types/models/channel";

export default function Test() {
    // Hook pour les snackbars
    const { showSnackbar } = useSnackbar();

    // État nécessaire pour l'exemple de l'input contrôlé (Email)
    const [email, setEmail] = useState('');

    // États pour gérer l'ouverture des modales
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [isDeleteOpen, setIsDeleteOpen] = useState(false);

    // State pour le Dropdown
    const [role, setRole] = useState<string | number>('');

    // Définition des options
    const roles: Option[] = Object.keys(Role).filter((key) => isNaN(Number(key))).map((key) => ({
        label: {
            OWNER: "Propriétaire",
            ADMIN: "Administrateur",
            MEMBER: "Membre",
        }[key as keyof typeof Role],
        value: key.toLowerCase(),
    }));

    const channelsList: Channel[] = [
        {
            id: 1,
            server_id: 1,
            name: 'Général',
            created_at: new Date(),
        },
        {
            id: 2,
            server_id: 2,
            name: 'information',
            created_at: new Date(),
        },
        {
            id: 3,
            server_id: 3,
            name: 'Invites',
            created_at: new Date(),
        },

    ]

    const handleClick = (message: string) => {
        console.log(message);
    };

    return (
        <div className="flex flex-col items-center justify-center gap-10 p-10 w-full">

            <h1 className="text-3xl font-bold text-white">
                Component Playground
            </h1>

            {/* --- SECTION 1 : BOUTONS & SPINNER --- */}
            <section className="flex flex-col items-center gap-5 w-full max-w-md p-6 bg-gray-300 rounded-xl shadow-sm border border-gray-200">
                <h2 className="text-lg font-semibold text-white uppercase tracking-wider">Actions</h2>

                <Button
                    variant={'primary'}
                    width="200px"
                    height="50px"
                    onClick={() => handleClick("Custom size button clicked")}
                >
                    Custom Size
                </Button>

                <LoadingSpinner size="lg" />
            </section>

            {/* --- SECTION 2 : FORMULAIRES (Tes inputs) --- */}
            <section className="flex flex-col gap-6 w-full max-w-md p-6 bg-gray-300 rounded-xl shadow-sm border border-gray-200">
                <h2 className="text-lg font-semibold text-center text-white uppercase tracking-wider">Formulaire</h2>

                {/* 1. Input Simple */}
                <Input
                    label="Nom complet"
                    placeholder="Jean Dupont"
                />

                {/* 2. Input avec Icône (Email contrôlé via useState) */}
                <Input
                    label="Email"
                    type="email"
                    placeholder="exemple@mail.com"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    // Icône SVG (Enveloppe)
                    leftIcon={
                        <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                        </svg>
                    }
                />


                {/* 3. Input avec Erreur */}
                <Input
                    label="Mot de passe"
                    type="password"
                    defaultValue="123" // J'utilise defaultValue ici pour l'exemple sans state
                    error="Le mot de passe est trop court."
                    rightIcon={
                        <svg className="w-5 h-5 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    }
                />

                {/* 4. Input avec Helper text */}
                <Input
                    label="Pseudo"
                    placeholder="@pseudo"
                    helperText="Sera visible par tout le monde."
                />

                {/* 5. Le Dropdown */}
                <Dropdown
                    label="Rôle utilisateur"
                    placeholder="Choisir un rôle..."
                    options={roles}
                    value={role}
                    onChange={(val) => setRole(val)}
                    helperText="Définit les permissions d'accès."
                />

                {/* 6. Dropdown avec erreur */}
                <Dropdown
                    label="Pays (Exemple erreur)"
                    options={[{ label: 'France', value: 'fr' }]}
                    value=""
                    onChange={() => { }}
                    error="Veuillez sélectionner un pays."
                />

            </section>

            {/* --- SECTION 3 : DIALOGS / MODALES --- */}
            <section className="flex flex-col items-center gap-5 w-full max-w-md p-6 bg-gray-300 rounded-xl shadow-sm border border-gray-200">
                <h2 className="text-lg font-semibold text-white uppercase tracking-wider">Dialogs</h2>

                <div className="flex gap-4">
                    {/* Bouton pour ouvrir une modale simple */}
                    <Button onClick={() => setIsDialogOpen(true)}>
                        Ouvrir Info
                    </Button>

                    {/* Bouton pour ouvrir une modale de confirmation (Rouge) */}
                    <Button variant="danger" onClick={() => setIsDeleteOpen(true)}>
                        Supprimer
                    </Button>
                </div>
            </section>

            {/* 1. Modale d'Information */}
            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Bienvenue !"
                description="Ceci est une description optionnelle en dessous du titre."
                footer={
                    <Button variant={"primary"} onClick={() => setIsDialogOpen(false)}>Compris</Button>
                }
            >
                <p className="text-white">
                    Ceci est le corps de la modale. Tu peux y mettre ce que tu veux :
                    du texte, des images, ou même des formulaires.
                </p>
                <div className="mt-4 p-4 bg-blurple/20 text-blurple rounded-lg border border-blurple/30">
                    Info : Tu peux fermer en cliquant sur le fond noir ou Echap.
                </div>
            </Dialog>

            {/* 2. Modale de Suppression (Action critique) */}
            <Dialog
                isOpen={isDeleteOpen}
                onClose={() => setIsDeleteOpen(false)}
                title="Confirmer la suppression"
                preventCloseOnOverlay={true} // Oblige l'utilisateur à choisir
                footer={
                    <>
                        <Button variant="outline" onClick={() => setIsDeleteOpen(false)}>
                            Annuler
                        </Button>
                        <Button variant="danger" onClick={() => {
                            alert("Supprimé !");
                            setIsDeleteOpen(false);
                        }}>
                            Confirmer la suppression
                        </Button>
                    </>
                }
            >
                <p className="text-gray-100">Êtes-vous sûr de vouloir supprimer cet élément ? Cette action est irréversible.</p>
            </Dialog>

            {/* --- SECTION 5 : SNACKBARS --- */}
            <section className="flex flex-col items-center gap-5 w-full max-w-md p-6 bg-gray-300 rounded-xl shadow-sm border border-gray-200">
                <h2 className="text-lg font-semibold text-white uppercase tracking-wider">Snackbars</h2>

                <Button
                    variant="danger"
                    onClick={() => showSnackbar({
                        message: "Erreur : Quelque chose s'est mal passé !",
                        severity: "error",
                        position: { vertical: "top", horizontal: "center" }
                    })}
                >
                    Erreur en haut au centre
                </Button>

                <Button
                    variant="primary"
                    onClick={() => showSnackbar({
                        message: "Opération réussie !",
                        severity: "success",
                        position: { vertical: "bottom", horizontal: "center" }
                    })}
                >
                    Succès en bas au centre
                </Button>
            </section>



            <section>
                <ChannelList channels={channelsList}></ChannelList>
            </section>

        </div>

    );
}