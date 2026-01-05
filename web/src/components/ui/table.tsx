import { cn } from "@/lib/utils";
import { Component, ComponentProps } from "solid-js";

export const Table: Component<ComponentProps<"table">> = (props) => (
    <table {...props} class={cn("w-full", props.class)} />
);

export const TableHead: Component<ComponentProps<"thead">> = (props) => (
    <thead {...props} class={cn("", props.class)} />
);

export const TableRow: Component<ComponentProps<"tr">> = (props) => (
    <tr
        {...props}
        class={cn("hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors", props.class)}
    />
);

export const TableHeader: Component<ComponentProps<"th">> = (props) => (
    <th
        {...props}
        class={cn(
            "text-muted-foreground h-12 px-4 text-left align-middle font-medium [&:has([role=checkbox])]:pr-0",
            props.class,
        )}
    />
);

export const TableBody: Component<ComponentProps<"tbody">> = (props) => (
    <tbody {...props} class={cn("[&_tr:last-child]:border-0", props.class)} />
);

export const TableCell: Component<ComponentProps<"td">> = (props) => (
    <td {...props} class={cn("p-4 align-middle [&:has([role=checkbox])]:pr-0", props.class)} />
);
