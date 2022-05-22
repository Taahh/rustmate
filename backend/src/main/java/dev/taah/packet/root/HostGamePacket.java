package dev.taah.packet.root;

import com.google.gson.GsonBuilder;
import dev.taah.connection.PlayerConnection;
import dev.taah.data.GameOptionsData;
import dev.taah.packet.ReliablePacket;
import dev.taah.player.InnerPlayer;
import dev.taah.server.GameRoom;
import dev.taah.server.GameRoomManager;
import dev.taah.util.GameCode;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import org.apache.commons.lang3.RandomStringUtils;

import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class HostGamePacket extends ReliablePacket<HostGamePacket>
{
    private GameOptionsData gameOptionsData;
    private GameCode gameCode;

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        this.gameOptionsData = GameOptionsData.deserialize(buffer);
        System.out.println(new GsonBuilder().setPrettyPrinting().create().toJson(gameOptionsData));
        System.out.println("Cross-play Flags: " + buffer.readUInt32());
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
//        buffer.writeInt32(GameCode.codeToInt("REDSUS"));
        HazelMessage hazelMessage = HazelMessage.start(0x00);
//        buffer.writeByteArray(hazelMessage.getPayload().getByteArray());
        hazelMessage.getPayload().writeInt32(this.gameCode.getGameId());
        hazelMessage.endMessage();
        buffer.writeBytes(hazelMessage.getPayload().getByteArray());
    }

    @Override
    public void processPacket(HostGamePacket packet, PlayerConnection connection)
    {
        try
        {
            System.out.println("PROCESSING HOST GAME PACKET");
//            String code = RandomStringUtils.randomAlphabetic(6);
//            String code = "REDSUS";
            packet.gameCode = GameCode.generateCode();
            while (GameRoomManager.gameRoomExists(packet.gameCode))
            {
                packet.gameCode = GameCode.generateCode();
            }
            GameRoom server = new GameRoom(packet.gameCode);
            server.setHostUuid(connection.getUuid());
            server.setGameOptionsData(packet.gameOptionsData);
            InnerPlayer player = new InnerPlayer(connection.getUuid(), connection, server);
            player.setId(server.getPlayers().size() + 1);
            server.getPlayers().put(player.getUuid(), player);
            GameRoomManager.addGameRoom(server);
            connection.sendPacket(packet);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
