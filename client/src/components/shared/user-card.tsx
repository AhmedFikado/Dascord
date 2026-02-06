
export default function UserCard({ username }: { username?: string }) {

    const getInitials = (username?: string) => {
        return username?.charAt(0).toUpperCase() || '?';
    };

    const getColorFromUsername = (username?: string) => {
        if (!username) return '#5865F2';
        
        const colors = [
            '#5865F2', // bleue
            '#47c16e', // vert
            '#ED4245', // rouge
            '#e9d253', // jaune
            '#e71d89', // rose
            '#c3511c', // orange
            '#01b9da', // cyan
            '#9B59B6', // violet
            '#870606', // rouge foncé
            '#1ABC9C', // turquoise
        ];
        
        if (username === 'Système') return '#3e3e3f';

        let hash = 0;
        for (let i = 0; i < username.length; i++) {
            hash = username.charCodeAt(i) + ((hash * 31) - hash);
        }
        
        return colors[Math.abs(hash) % colors.length];
    };

    return (
        <div className="flex-shrink-0">
            <div 
                className="w-10 h-10 rounded-full flex items-center justify-center text-white font-semibold"
                style={{ backgroundColor: getColorFromUsername(username) }}
            >
                {getInitials(username)}
            </div>
        </div>

    );
}