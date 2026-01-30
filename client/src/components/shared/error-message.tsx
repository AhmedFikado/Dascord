// components/shared/error-message.tsx
'use client';

import { createContext, useContext, useState, ReactNode } from 'react';
import Snackbar from '@mui/material/Snackbar';
import Alert, { AlertColor } from '@mui/material/Alert';

interface SnackbarConfig {
    message: string;
    severity?: AlertColor;
    duration?: number;
    position?: {
        vertical: 'top' | 'bottom';
        horizontal: 'left' | 'center' | 'right';
    };
}

interface SnackbarContextType {
    showSnackbar: (config: SnackbarConfig) => void;
}

const SnackbarContext = createContext<SnackbarContextType | undefined>(undefined);

export function SnackbarProvider({ children }: { children: ReactNode }) {
    const [open, setOpen] = useState(false);
    const [config, setConfig] = useState<SnackbarConfig>({
        message: '',
        severity: 'info',
        duration: 5000,
        position: { vertical: 'bottom', horizontal: 'center' },
    });

    const showSnackbar = (newConfig: SnackbarConfig) => {
        setConfig({
            severity: 'info',
            duration: 5000,
            position: { vertical: 'bottom', horizontal: 'center' },
            ...newConfig,
        });
        setOpen(true);
    };

    const handleClose = (_event?: React.SyntheticEvent | Event, reason?: string) => {
        if (reason === 'clickaway') {
            return;
        }
        setOpen(false);
    };

    return (
        <SnackbarContext.Provider value={{ showSnackbar }}>
            {children}
            <Snackbar
                open={open}
                autoHideDuration={config.duration}
                onClose={handleClose}
                anchorOrigin={config.position}
            >
                <Alert
                    onClose={handleClose}
                    severity={config.severity}
                    variant="filled"
                    sx={{ width: '100%' }}
                >
                    {config.message}
                </Alert>
            </Snackbar>
        </SnackbarContext.Provider>
    );
}

export function useSnackbar() {
    const context = useContext(SnackbarContext);

    // Si pas de context, retourne un fallback au lieu de throw
    if (!context) {
        // En développement, log l'avertissement
        if (process.env.NODE_ENV === 'development') {
            console.warn('⚠️ useSnackbar appelé en dehors du SnackbarProvider');
        }

        // Retourne un mock qui ne fait rien
        return {
            showSnackbar: (config: SnackbarConfig) => {
                console.log('Snackbar (non-connecté):', config.message);
            }
        };
    }

    return context;
}