from loid import Event, Neuron, DefaultAgent

neuron: Neuron = DefaultAgent.neuron(
    name="Server Status Check",
    description="Checks the status of the server",
)

@neuron.action
def check_server_status(event: Event, ip: str):
    """"""
    is_server_up = ...

    if is_server_up:
        event.goals.new("resolve alert")
    else:
        event.goals.new("restart server")
