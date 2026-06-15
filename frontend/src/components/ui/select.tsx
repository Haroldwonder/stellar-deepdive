"use client";

import * as React from "react";
import { Check, ChevronDown } from "lucide-react";
import { cn } from "@/lib/utils";

interface SelectContextValue {
  value: string;
  open: boolean;
  setOpen: (open: boolean) => void;
  select: (value: string) => void;
  registerLabel: (value: string, label: React.ReactNode) => void;
  labels: Map<string, React.ReactNode>;
}

const SelectContext = React.createContext<SelectContextValue | null>(null);

function useSelectContext(component: string): SelectContextValue {
  const ctx = React.useContext(SelectContext);
  if (!ctx) {
    throw new Error(`${component} must be used within <Select>`);
  }
  return ctx;
}

interface SelectProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  children: React.ReactNode;
}

function Select({ value, defaultValue, onValueChange, children }: SelectProps) {
  const [internal, setInternal] = React.useState(defaultValue ?? "");
  const current = value !== undefined ? value : internal;
  const [open, setOpen] = React.useState(false);
  const containerRef = React.useRef<HTMLDivElement>(null);
  const labelsRef = React.useRef<Map<string, React.ReactNode>>(new Map());
  const [, forceUpdate] = React.useReducer((c) => c + 1, 0);

  const select = React.useCallback(
    (next: string) => {
      if (value === undefined) setInternal(next);
      onValueChange?.(next);
      setOpen(false);
    },
    [value, onValueChange],
  );

  const registerLabel = React.useCallback((itemValue: string, label: React.ReactNode) => {
    if (!labelsRef.current.has(itemValue)) {
      labelsRef.current.set(itemValue, label);
      forceUpdate();
    }
  }, []);

  React.useEffect(() => {
    if (!open) return;
    const handleClick = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [open]);

  return (
    <SelectContext.Provider
      value={{ value: current, open, setOpen, select, registerLabel, labels: labelsRef.current }}
    >
      <div ref={containerRef} className="relative">
        {children}
      </div>
    </SelectContext.Provider>
  );
}

interface SelectTriggerProps {
  className?: string;
  children: React.ReactNode;
}

function SelectTrigger({ className, children }: SelectTriggerProps) {
  const ctx = useSelectContext("SelectTrigger");
  return (
    <button
      type="button"
      role="combobox"
      aria-expanded={ctx.open}
      onClick={() => ctx.setOpen(!ctx.open)}
      className={cn(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
    >
      {children}
      <ChevronDown className="h-4 w-4 opacity-50" />
    </button>
  );
}

function SelectValue({ placeholder }: { placeholder?: string }) {
  const ctx = useSelectContext("SelectValue");
  const label = ctx.value ? ctx.labels.get(ctx.value) : undefined;
  if (label) return <span className="truncate">{label}</span>;
  return <span className="truncate text-muted-foreground">{placeholder}</span>;
}

function SelectContent({ className, children }: { className?: string; children: React.ReactNode }) {
  const ctx = useSelectContext("SelectContent");
  // Always rendered (hidden when closed) so items can register their labels.
  return (
    <div
      role="listbox"
      className={cn(
        "absolute z-50 mt-1 max-h-60 w-full min-w-[8rem] overflow-auto rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md",
        ctx.open ? "block" : "hidden",
        className,
      )}
    >
      {children}
    </div>
  );
}

interface SelectItemProps {
  value: string;
  className?: string;
  children: React.ReactNode;
}

function SelectItem({ value, className, children }: SelectItemProps) {
  const ctx = useSelectContext("SelectItem");
  const selected = ctx.value === value;

  React.useEffect(() => {
    ctx.registerLabel(value, children);
  }, [ctx, value, children]);

  return (
    <button
      type="button"
      role="option"
      aria-selected={selected}
      onClick={() => ctx.select(value)}
      className={cn(
        "relative flex w-full cursor-pointer select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm text-left outline-none hover:bg-accent hover:text-accent-foreground",
        className,
      )}
    >
      {selected && (
        <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
          <Check className="h-4 w-4" />
        </span>
      )}
      {children}
    </button>
  );
}

export { Select, SelectTrigger, SelectValue, SelectContent, SelectItem };
