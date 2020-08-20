defmodule RaceElixir.Endpoint do
  use Plug.Router

  plug(Plug.Logger)

  plug(:match)

  plug(Plug.Parsers, parsers: [:json], json_decoder: Jason)

  plug(:dispatch)

  get "/generate_primes" do
    %{"number" => number} = conn.params

    {int, ""} = Integer.parse(number)
    {:ok, primes} = NativePrimesGenerator.generate_primes(int)

    send_resp(conn, 200, primes)
  end

  get "/" do
    send_resp(conn, 200, "Boo!")
  end

  post "/events" do
    {status, body} =
      case conn.body_params do
        %{"events" => events} -> {200, process_events(events)}
        _ -> {422, missing_events()}
      end

    send_resp(conn, status, body)
  end

  defp process_events(events) when is_list(events) do
    Jason.encode!(%{response: "Received Events!"})
  end

  defp process_events(_) do
    Jason.encode!(%{response: "Please Send Some Events!"})
  end

  defp missing_events do
    Jason.encode!(%{error: "Expected Payload: { 'events': [...] }"})
  end

  match _ do
    send_resp(conn, 404, "oops... Nothing here :(")
  end
end
