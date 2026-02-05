'use client';

import React, { useState, useRef, useEffect } from 'react';
import { createPortal } from 'react-dom';

export interface Option {
    label: string;
    value: string | number;
}

interface DropdownProps {
    label?: string;
    options: Option[];
    value?: string | number;
    onChange: (val: any) => void;
    placeholder?: string;
    error?: string;
    helperText?: string;
    disabled?: boolean;
    className?: string;
    width?: string;
}

export const Dropdown: React.FC<DropdownProps> = ({
    label,
    options,
    value,
    onChange,
    placeholder = 'Sélectionner...',
    error,
    helperText,
    disabled = false,
    className = '',
    width = '100%',
}) => {
    const [isOpen, setIsOpen] = useState(false);
    const [menuPosition, setMenuPosition] = useState({ top: 0, left: 0, width: 0 });
    const containerRef = useRef<HTMLDivElement>(null);
    const buttonRef = useRef<HTMLButtonElement>(null);

    const selectedOption = options.find((opt) => opt.value === value);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => document.removeEventListener('mousedown', handleClickOutside);
    }, []);

    useEffect(() => {
        if (isOpen && buttonRef.current) {
            const rect = buttonRef.current.getBoundingClientRect();
            setMenuPosition({
                top: rect.bottom + window.scrollY,
                left: rect.left + window.scrollX,
                width: rect.width,
            });
        }
    }, [isOpen]);

    const handleSelect = (option: Option) => {
        if (disabled) return;
        onChange(option.value);
        setIsOpen(false);
    };

    return (
        <div
            className={`flex flex-col gap-1.5 ${className}`}
            style={{ width }}
            ref={containerRef}
        >
            {label && (
                <label className="text-sm font-medium text-white">
                    {label}
                </label>
            )}

            <div className="relative">
                <button
                    ref={buttonRef}
                    type="button"
                    onClick={() => !disabled && setIsOpen(!isOpen)}
                    disabled={disabled}
                    className={`
                        relative w-full text-left rounded-lg border px-3 py-2 text-sm transition-all flex items-center justify-between
                        focus:outline-none focus:ring-2 disabled:opacity-50 bg-gray-400
                        
                        ${error
                            ? 'border-red focus:border-red focus:ring-red/50 text-red'
                            : 'border-gray-300 focus:border-blurple focus:ring-blurple/50 text-white'
                        }
                    `}
                >
                    <span className={`block truncate ${!selectedOption ? 'text-gray-200' : ''}`}>
                        {selectedOption ? selectedOption.label : placeholder}
                    </span>

                    <span className="pointer-events-none flex items-center pl-2">
                        <svg
                            className={`h-5 w-5 text-gray-200 transition-transform duration-200 ${isOpen ? 'rotate-180' : 'rotate-0'}`}
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path fillRule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clipRule="evenodd" />
                        </svg>
                    </span>
                </button>

                {isOpen && typeof window !== 'undefined' && createPortal(
                    <div
                        style={{
                            position: 'absolute',
                            top: `${menuPosition.top}px`,
                            left: `${menuPosition.left}px`,
                            width: `${menuPosition.width}px`,
                            zIndex: 9999,
                        }}
                        className="mt-1 max-h-60 overflow-auto rounded-md bg-gray-300 py-1 shadow-lg border border-gray-200 ring-1 ring-black ring-opacity-5 focus:outline-none"
                    >
                        {options.length > 0 ? (
                            options.map((option) => (
                                <div
                                    key={option.value}
                                    onClick={() => handleSelect(option)}
                                    className={`
                                        relative cursor-pointer select-none py-2 pl-3 pr-9 text-sm transition-colors
                                        hover:bg-blurple/80
                                        ${option.value === value
                                            ? 'bg-blurple text-white font-medium'
                                            : 'text-white'
                                        }
                                    `}
                                >
                                    <span className="block truncate">{option.label}</span>

                                    {option.value === value && (
                                        <span className="absolute inset-y-0 right-0 flex items-center pr-4 text-white">
                                            <svg className="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                                                <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                                            </svg>
                                        </span>
                                    )}
                                </div>
                            ))
                        ) : (
                            <div className="px-3 py-2 text-sm text-gray-200">Aucune option</div>
                        )}
                    </div>,
                    document.body
                )}
            </div>

            {error ? (
                <p className="text-xs text-red mt-0.4 ml-2">{error}</p>
            ) : helperText ? (
                <p className="text-xs text-gray-50 mt-0.4 ml-2">{helperText}</p>
            ) : null}
        </div>
    );
};