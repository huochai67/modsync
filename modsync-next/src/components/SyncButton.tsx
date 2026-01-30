
import { Button } from '@heroui/react';
import React from 'react';

interface SyncButtonProps {
    children?: React.ReactNode;
    onClicked: () => Promise<void>;
}

const SyncButton: React.FC<SyncButtonProps> = ({ onClicked, children }) => {
    const [disabled, setDisabled] = React.useState(false);

    const onClick = async () => {
        setDisabled(true);
        try {
            await onClicked();
        } catch (error) {
            console.error("Error in SyncButton:", error);
        } finally {
            setDisabled(false);
        }
    }

    return (
        <Button onClick={onClick} isDisabled={disabled}>{children}</Button>
    );
};

export default SyncButton;