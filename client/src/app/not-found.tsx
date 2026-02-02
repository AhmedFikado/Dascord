import Link from 'next/link';

export default function NotFound() {
	return (
		<div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
			<div className="text-center">
				<h1 className="text-9xl font-extrabold text-gray-200">404</h1>

				<div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 mt-2">
					<h2 className="text-3xl font-bold text-gray-800 mb-2">
						Page introuvable
					</h2>
					<p className="text-gray-600 mb-6 text-lg">
						Oups ! La page que vous cherchez semble avoir disparu.
					</p>

					<Link
						href="/"
						className="inline-block px-8 py-3 bg-blue-600 text-white font-semibold rounded-full shadow-md hover:bg-blue-700 transition duration-300 transform hover:scale-105"
					>
						Retour à l'accueil
					</Link>
				</div>
			</div>
		</div>
	);
}