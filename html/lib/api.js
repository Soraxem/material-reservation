

async function list_article() {
    const rawResponse = await fetch("/api/list-article/")
    
    const data = await rawResponse.json();
    return data;
}

async function create_article(name, description) {
    const rawResponse = await fetch('/api/create-article/', {
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

async function get_article(id) {
    const rawResponse = await fetch("/api/get-article/" + id);

    const data = await rawResponse.json();
    return data
}


async function list_position(article_id) {
    const rawResponse = await fetch("/api/list-position/" + article_id);

    const data = await rawResponse.json();
    return data
}

async function create_position(article_id, is_consumable, is_unique, amount, normal_amount = null, unique_name = null) {
    const rawResponse = await fetch('/api/create-position/', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            fk_article: article_id,
            is_consumable: is_consumable,
            is_unique: is_unique,
            amount: amount,
            normal_amount: normal_amount,
            unique_name: unique_name
        })
    });
}