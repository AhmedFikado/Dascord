'use client';

import React, { useEffect } from 'react';
import { Button } from './button';

interface DialogProps {
    isOpen: boolean;
    onClose: () => void;
    title?: string;
    description?: string;
    children: React.ReactNode;
    footer?: React.ReactNode; // Pour mettre des boutons "Valider / Annuler"
    size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
    preventCloseOnOverlay?: boolean; // Si true, oblige à cliquer sur la croix ou un bouton
}

export const Dialog: React.FC<DialogProps> = ({
                                                  isOpen,
                                                  onClose,
                                                  title,
                                                  description,
                                                  children,
                                                  footer,
                                                  size = 'md',
                                                  preventCloseOnOverlay = false,
                                              }) => {

    const sizeClasses = {
        sm: 'max-w-sm',
        md: 'max-w-md',
        lg: 'max-w-lg',
        xl: 'max-w-xl',
        full: 'max-w-[95vw]',
    };

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === 'Escape' && !preventCloseOnOverlay) {
                onClose();
            }
        };

        if (isOpen) {
            document.body.style.overflow = 'hidden';
            window.addEventListener('keydown', handleKeyDown);
        }

        return () => {
            document.body.style.overflow = 'unset';
            window.removeEventListener('keydown', handleKeyDown);
        };
    }, [isOpen, onClose, preventCloseOnOverlay]);

    if (!isOpen) return null;

    return (
        <div
            className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 transition-all"
            onClick={() => !preventCloseOnOverlay && onClose()}
        >
            <div
                className={`
                    bg-gray-300
                    w-full rounded-xl shadow-2xl 
                    transform transition-all scale-100
                    flex flex-col max-h-[90vh]
                    ${sizeClasses[size]}
                `}
                onClick={(e) => e.stopPropagation()}
                role="dialog"
                aria-modal="true"
            >

                {/* HEADER */}
                <div className="flex items-center justify-between p-5 border-b border-gray-200">
                    <div>
                        {title && <h3 className="text-lg font-semibold text-white">{title}</h3>}
                        {description && <p className="text-sm text-gray-100 mt-1">{description}</p>}
                    </div>

                    {/* Bouton Fermer (Croix) */}
                    <Button
                        onClick={onClose}
                        width={"50px"}
                        height={"50px"}
                        variant={"noBackground"}
                    >
                        <svg className="w-50 h-50" fill="none" viewBox="0 0 20 20" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </Button>
                </div>

                {/* BODY (Scrollable si le contenu est trop long) */}
                <div className="p-6 overflow-y-auto">
                    {children}
                </div>

                {/* FOOTER*/}
                {footer && (
                    <div className="p-5 border-gray-200 rounded-b-xl flex justify-end gap-3">
                        {footer}
                    </div>
                )}
            </div>


        </div>
    );
};