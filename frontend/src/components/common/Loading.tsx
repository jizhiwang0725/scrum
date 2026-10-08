import { LoaderCircle } from 'lucide-react'
import { cn } from '@/lib/utils'

export interface LoadingProps {
  message?: string
  className?: string
}

export function Loading({
  message = 'Loading...',
  className,
}: LoadingProps) {
  return (
    <div
      role="status"
      aria-live="polite"
      aria-atomic="true"
      className={cn(
        'flex flex-col items-center justify-center gap-4 px-6 py-12 text-center font-sans',
        className,
      )}
    >
      <div className="flex size-16 items-center justify-center rounded-full bg-primary/10 text-primary">
        <LoaderCircle
          aria-hidden="true"
          className="size-8 animate-spin motion-reduce:animate-none"
        />
      </div>
      <p className="max-w-md text-base break-words text-muted-foreground">
        {message}
      </p>
    </div>
  )
}
