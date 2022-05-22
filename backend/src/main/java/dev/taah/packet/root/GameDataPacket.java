package dev.taah.packet.root;

import dev.taah.connection.PlayerConnection;
import dev.taah.inner.InnerNetObject;
import dev.taah.packet.ReliablePacket;
import dev.taah.packet.root.gamedata.AbstractGameData;
import dev.taah.packet.root.gamedata.SpawnGameData;
import dev.taah.util.GameCode;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import lombok.SneakyThrows;

import java.util.HashMap;
import java.util.Map;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class GameDataPacket extends ReliablePacket<GameDataPacket>
{
    private static final Map<Integer, Class<? extends AbstractGameData>> GAME_DATA = new HashMap<>();

    public GameDataPacket()
    {
        this.registerGameData(0x04, SpawnGameData.class);
    }

    public void registerGameData(int id, Class<? extends AbstractGameData> clazz) {
        GAME_DATA.put(id, clazz);
    }

    @SneakyThrows
    public AbstractGameData getById(int id) {
        Class<? extends AbstractGameData> gameData = GAME_DATA.get(id);
        if (gameData == null)
        {
            return null;
        }
        return (AbstractGameData) gameData.getConstructors()[0].newInstance();
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        System.out.println("Game Code: " + new GameCode(buffer.readInt32()).getGameCode());
        HazelMessage message;
        while ((message = HazelMessage.read(buffer)) != null)
        {
            System.out.println("Game Data Packet ID: " + message.getTag());
            AbstractGameData gameData = getById(message.getTag());
            if (gameData != null)
            {
                gameData.deserialize(message.getPayload());
            }
            /*switch (message.getTag())
            {
                case 4 ->
                {
                    System.out.println("Net Obj Spawn ID: " + message.getPayload().readPackedUInt32());
                    System.out.println("Owner ID: " + message.getPayload().readPackedUInt32());
                    System.out.println("Flags: " + message.getPayload().readByte());
                    int components = (int) message.getPayload().readPackedUInt32();
                    System.out.println("Components: " + components);
                    for (int i = 0; i < components; i++)
                    {
                        InnerNetObject object = new InnerNetObject((int) buffer.readPackedUInt32(), HazelMessage.read(buffer));
                        System.out.println("Start Of Object: " + object.getData().getTag());
                        System.out.println("Inner Object ID: " + object.getNetId());
                    }
                }
                case 1 -> {
                    System.out.println("Data Net Object ID: " + message.getPayload().readPackedUInt32());
                }

            }*/
        }
    }

    @Override
    public void processPacket(GameDataPacket packet, PlayerConnection connection)
    {
    }
}
