export async function subscribe(manager, api_key) {
    let subscription = await manager.subscribe({
        applicationServerKey: api_key,
        userVisibleOnly: true
    });

    console.log("subscription", subscription);
    return subscription;
}