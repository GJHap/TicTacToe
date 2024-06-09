export default function ({
    onClick,
    value,
}: {
    onClick: () => void
    value: string
}) {
    return (
        <button className="w-full h-full" onClick={onClick}>
            {value}
        </button>
    )
}
