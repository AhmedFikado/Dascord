
export default function UserCard({ user }: { user: { username: string } }) {

    const getInitials = (username: string) => {
        return username.charAt(0).toUpperCase();
    };

    return (
        <div className="flex-shrink-0">
            <div className="w-10 h-10 rounded-full bg-blurple flex items-center justify-center text-white font-semibold">
                {getInitials(user.username)}
            </div>
        </div>

    );
}