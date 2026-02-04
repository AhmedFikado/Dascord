export default function TypingIndicator({ username }: { username: string }) {
    return (
        <div>
            <p className="text-gray-light ml-6 text-xs mb-1">{username} est en train d'écrire...</p>
        </div>
    );
}