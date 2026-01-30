import { User } from "@/types/models/user";


export default function TypingIndicator({ user }: { user: User }) {
    return (
        <div>
            <p className="text-gray-light ml-6 text-xs mb-1">{user.username} est entrain d'écrire...</p>
        </div>
    );
}