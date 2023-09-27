using Discord;
using Discord.WebSocket;

namespace Vertex
{
    class Program
    {
        private DiscordSocketClient? _client;
        private readonly string _token = "MTE1NjM2NjQyMjE0NzA4MDM0Mw.GbWvv7.XbIbbp9M9wfp0sDQwtDHSrBZRFDWSJWfhIyURk";

        public static Task Main(string[] args) => new Program().MainAsync();

        public async Task MainAsync()
        {
            _client = new DiscordSocketClient();

            _client.Log += Log;
            _client.Ready += ReadyAsync;

            // Some alternative options would be to keep your token in an Environment Variable or a standalone file.
            // var token = Environment.GetEnvironmentVariable("NameOfYourEnvironmentVariable");
            // var token = File.ReadAllText("token.txt");
            // var token = JsonConvert.DeserializeObject<AConfigurationClass>(File.ReadAllText("config.json")).Token;

            await _client.LoginAsync(TokenType.Bot, _token);
            await _client.StartAsync();

            // Block this task until the program is closed.
            await Task.Delay(-1);
        }

        private Task Log(LogMessage msg)
        {
            Console.WriteLine(msg.ToString());
            return Task.CompletedTask;
        }

        private async Task<Task> ReadyAsync()
        {
            if (_client != null)
            {
                var defaultChannel = _client.GetGuild(1107146391039508480)?.TextChannels.FirstOrDefault();

                if (defaultChannel != null)
                {
                    try
                    {
                        await defaultChannel.SendMessageAsync("Hello World!");
                        Console.WriteLine("Message sent!");
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine($"Message not sent! {ex.Message}");
                    }
                }
            }

            return Task.CompletedTask;
        }
    }
}