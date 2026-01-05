import { cn } from "@/lib/utils";
import { Component, ComponentProps, createEffect, createSignal } from "solid-js";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table";
import { aramsStore, setAramsStore } from "@/stores/arams";

export const MachineSection: Component<ComponentProps<"section">> = ({ class: className, ...props }) => {
    const [accumulator, setAccumulator] = createSignal<number | undefined>(undefined);
    const [registers, setRegisters] = createSignal<Map<number, number>>(new Map());

    createEffect(() => {
        const machine = aramsStore.machine;
        if (machine) {
            setAccumulator(machine.accumulator);
            setRegisters(machine.registers);
        }
    }, [aramsStore.machine]);

    createEffect(() => {
        setAramsStore("machine", undefined);
        setAccumulator(undefined);
        const preseed = aramsStore.preseedRegisters;
        if (preseed) {
            setRegisters(new Map(preseed));
        }
    }, [aramsStore.preseedRegisters]);

    const updatePreseedRegisters = (reg: number, value: number) => {
        setAccumulator(undefined);
        setAramsStore("preseedRegisters", (prev) => {
            const newPreseed = new Map(prev);
            newPreseed.set(reg, value);
            return newPreseed;
        });
        setAramsStore("machine", undefined);
    };

    return (
        <section class={cn("border-ring ring-ring/50 h-full rounded-md p-2 ring-2", className)} {...props}>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableHeader>Register</TableHeader>
                        <TableHeader>Value</TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {accumulator() !== undefined && (
                        <TableRow>
                            <TableCell>ACC</TableCell>
                            <TableCell>{accumulator()}</TableCell>
                        </TableRow>
                    )}
                    {Array.from(registers().entries())
                        .sort(([a], [b]) => a - b)
                        .map(([reg, value]) => (
                            <TableRow>
                                <TableCell>R{reg}</TableCell>
                                <TableCell>
                                    <input
                                        type="number"
                                        value={value}
                                        class="w-full bg-transparent outline-none"
                                        onInput={(e) =>
                                            updatePreseedRegisters(reg, parseInt(e.currentTarget.value, 10) || 0)
                                        }
                                    />
                                </TableCell>
                            </TableRow>
                        ))}
                </TableBody>
            </Table>
        </section>
    );
};
