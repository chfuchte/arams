import { createEffect, createMemo, createSignal, onMount, type Component } from "solid-js";
import { Line } from "./line";

export const CodeEditor: Component<{
    code: () => string;
    setCode: (code: string) => void;
    errors: () => Record<number, string[]> | undefined;
}> = ({ code, setCode, errors }) => {
    const [visibleLinesBounds, setVisibleLinesBounds] = createSignal<[number, number]>([0, 0]);

    const lines = createMemo(() => code().split("\n"));
    const visibleLines = createMemo(() => {
        const [start, end] = visibleLinesBounds();
        return lines().slice(start, end);
    });

    let textareaRef: HTMLTextAreaElement | undefined;
    let previewRef: HTMLDivElement | undefined;

    onMount(() => {
        syncScroll();
    });

    createEffect(() => {
        syncScroll();
    }, [code]);

    const handleKeyDown = (e: KeyboardEvent) => {
        if (e.key === "Tab") {
            e.preventDefault();
            const target = e.target as HTMLTextAreaElement;
            const start = target.selectionStart;
            const end = target.selectionEnd;
            const newValue = target.value.substring(0, start) + "\t" + target.value.substring(end);
            setCode(newValue);
            requestAnimationFrame(() => {
                target.selectionStart = target.selectionEnd = start + 1;
            });
        }
    };

    const handleInput = (e: InputEvent) => {
        setCode((e.target as HTMLTextAreaElement).value);
    };

    const syncScroll = () => {
        let start = 0;
        let end = 0;

        if (textareaRef) {
            if (previewRef) {
                previewRef.scrollTop = textareaRef.scrollTop;
                previewRef.scrollLeft = textareaRef.scrollLeft;
            }

            const scroll = textareaRef.scrollTop;
            const height = textareaRef.clientHeight;

            let lineHeight: number;
            if (!textareaRef) {
                lineHeight = 20;
            } else {
                const style = getComputedStyle(textareaRef);
                lineHeight = parseFloat(style.lineHeight || style.fontSize);
            }

            const buffer = 10;
            start = Math.max(0, Math.floor(scroll / lineHeight) - buffer);
            end = Math.min(lines().length, Math.ceil((scroll + height) / lineHeight) + buffer);
        }

        setVisibleLinesBounds([start, end]);
    };

    return (
        <div class="relative h-full w-full overflow-hidden font-mono text-sm leading-5">
            <div
                ref={previewRef}
                class="pointer-events-none absolute top-0 left-0 z-0 flex h-full w-full overflow-hidden p-2 whitespace-pre">
                <div class="text-muted-foreground h-full w-12 shrink-0 pr-4 text-right select-none">
                    {(() => {
                        const [start, _] = visibleLinesBounds();
                        if (visibleLines().length === 0) return <pre aria-hidden>{start + 1}</pre>;
                        return visibleLines().map((_, i) => <pre aria-hidden>{start + i + 1}</pre>);
                    })()}
                </div>

                <div class="flex-1 pl-4">
                    {visibleLines().map((line, i) => (
                        <Line code={line} errors={errors()?.[i + 1] || []} />
                    ))}
                </div>
            </div>

            <textarea
                ref={textareaRef}
                autofocus
                class="border-muted caret-muted-foreground absolute top-0 left-0 z-10 ml-12 h-full w-[calc(100%-3rem)] resize-none overflow-auto border-l bg-transparent p-2 pl-6 leading-5 font-medium text-transparent outline-none"
                value={code()}
                onInput={handleInput}
                onScroll={syncScroll}
                onKeyDown={handleKeyDown}
                spellcheck={false}
            />
        </div>
    );
};
