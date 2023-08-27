import random
import string
from datetime import datetime
from fastapi import FastAPI


app = FastAPI()


@app.get("/")
async def root():
    return {"message": "Hello World"}


def generate_random_string(ln: int = 10):
    return "".join(random.choice(string.ascii_letters) for _ in range(ln))


def generated_pseudo_event(source_name: str):
    return {
        "source": source_name,
        "name": generate_random_string(),
        "timestamp": str(datetime.now().isoformat()),
    }


@app.get("/{source}/events")
async def events(source: str):
    return [generated_pseudo_event(source) for _ in range(10)]


@app.get("/{source}/event")
async def event(source: str):
    return generated_pseudo_event(source)
