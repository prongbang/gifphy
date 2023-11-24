initial_tailwind:
	bun install flowbite-svelte flowbite -d
	bun install tailwindcss -d
	npx svelte-add@latest tailwindcss

install:
	bun install 

dev:
	bun run dev

build:
	bun run build

preview: 
	bun run preview

check:
	bun run check

tauri:
	bun run tauri