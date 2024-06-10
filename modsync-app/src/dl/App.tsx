import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { mb_error, mb_info } from "../messagebox";
import { Box, LinearProgress, LinearProgressProps, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography } from "@mui/material";
import "../base.css"

function LinearProgressWithLabel(props: LinearProgressProps & { value: number }) {
    return (
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Box sx={{ width: '100%', mr: 1 }}>
                <LinearProgress variant="determinate" {...props} />
            </Box>
            <Box sx={{ minWidth: 35 }}>
                <Typography variant="body2" color="text.secondary">{`${Math.round(
                    props.value,
                )}%`}</Typography>
            </Box>
        </Box>
    );
}

type TaskInfo = {
    totalsize: number,
    downloadsize: number,
    name: string,
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

function App() {
    const [tasklist, settasklist] = useState(new Array<TaskInfo>());
    const [count, setCount] = useState(1);
    useEffect(() => {
        invoke<TaskInfo[]>('get_tasks').then((value) => {
            sleep(50).then(() => {
                settasklist(value);
                setCount((c) => c + 1);
                if (value.length == 0) {
                    mb_info("done!");
                    window.location.replace('/')
                }
            });
        }).catch(mb_error);
    }, []);

    return (
        <main className="w-screen h-screen rounded-xl border-4">
            <div className="flex flex-col h-full divide-y-4">
                <div className="grow w-full overflow-auto">
                    <TableContainer component={Paper}>
                        <Table id="table">
                            <TableHead>
                                <TableRow id="header">
                                    <TableCell id="name">Name</TableCell>
                                    <TableCell id="progress" align='left'>Progress</TableCell>
                                </TableRow>
                            </TableHead>
                            <TableBody id={count.toString()}>
                                {tasklist.map((row) => {
                                    return (
                                        <TableRow>
                                            <TableCell align="left" className=" w-2/5">{row.name}</TableCell>
                                            <TableCell align="left"><LinearProgressWithLabel value={(row.downloadsize / row.totalsize) * 100} /></TableCell>
                                        </TableRow>)
                                })}
                            </TableBody>
                        </Table>
                    </TableContainer>
                </div>
            </div>
        </main>
    );
}

export default App;
