from bindings_host import Root
from wasmtime import Config, Engine, Store

config = Config()
config.cache = True
engine = Engine(config)
store = Store(engine)
gift_suggestions_generator = Root(store)
print(f"component says: {gift_suggestions_generator.suggest(store, 'A', 1, 'B')}")