

async function list_article() {
    const rawResponse = await fetch("/api/list-article")
    
    const data = await rawResponse.json();
    return data;
}

async function create_article(name, description) {
    const rawResponse = await fetch('/api/create-article', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            name: name,
            description: description
        })
    });
}
