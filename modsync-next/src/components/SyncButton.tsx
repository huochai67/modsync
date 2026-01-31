
import { Button } from '@heroui/react';
import React from 'react';

interface SyncButtonProps {
    children?: React.ReactNode;
    isDisabled?: boolean;
    onClicked: () => Promise<void>;
}

const SyncButton: React.FC<SyncButtonProps> = ({ onClicked, isDisabled, children }) => {
    const [__disabled, setDisabled] = React.useState(isDisabled);

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
        <Button onClick={onClick} isDisabled={__disabled}>{children}</Button>
    );
};

export default SyncButton;