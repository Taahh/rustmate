package dev.taah.packet.root;

import com.google.gson.GsonBuilder;
import dev.taah.connection.PlayerConnection;
import dev.taah.data.GameOptionsData;
import dev.taah.packet.DisconnectPacket;
import dev.taah.packet.ReliablePacket;
import dev.taah.player.InnerPlayer;
import dev.taah.server.GameRoom;
import dev.taah.server.GameRoomManager;
import dev.taah.util.GameCode;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class JoinGamePacket extends ReliablePacket<JoinGamePacket>
{
    private InnerPlayer joining;
    private GameCode gameCode;
    private GameRoom gameRoom;

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        System.out.println("Got Game Code: " + (this.gameCode = new GameCode(buffer.readInt32())).getGameCode());
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
        HazelMessage hazelMessage = HazelMessage.start(0x01);
        hazelMessage.getPayload().writeInt32(this.gameRoom.getGameCode().getGameId());
        hazelMessage.getPayload().writeInt32(this.joining.getId());
        hazelMessage.getPayload().writeInt32(this.gameRoom.getPlayers().get(this.gameRoom.getHostUuid()).getId());
        this.joining.getConnection().getPlatformData().serialize(hazelMessage.getPayload());
        hazelMessage.getPayload().writePackedInt32(0);
        hazelMessage.getPayload().writeString("none");
        hazelMessage.getPayload().writeString("none");
        hazelMessage.endMessage();
        buffer.writeBytes(hazelMessage.getPayload().getByteArray());
    }

    @Override
    public void processPacket(JoinGamePacket packet, PlayerConnection connection)
    {
        System.out.println("PROCESSING JOIN GAME PACKET");
        GameCode gameCode1 = packet.gameCode;
        System.out.println(gameCode1.getGameCode());
        GameRoom gameRoom = GameRoomManager.getGameRoom(gameCode1);
        System.out.println("Game Room? " + (gameRoom != null));
        if (gameRoom != null)
        {
            System.out.println("Found Game Room !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            InnerPlayer player;
            if (!connection.getUuid().equals(gameRoom.getHostUuid()))
            {
                player = new InnerPlayer(connection.getUuid(), connection, gameRoom);
                player.setId(gameRoom.getPlayers().size() + 1);
                gameRoom.getPlayers().put(player.getUuid(), player);
                packet.joining = player;
                packet.gameRoom = gameRoom;
                gameRoom.sendPacket(packet, connection.getUuid());
            } else {
                player = gameRoom.getPlayers().get(connection.getUuid());
            }

            JoinedGamePacket pckt = new JoinedGamePacket();
            pckt.setNonce(packet.getNonce());
            pckt.setGameCode(packet.gameCode);
            pckt.setGameRoom(gameRoom);
            pckt.setJoining(player);
            connection.sendPacket(pckt);
            System.out.println("Sent joined game");
        } else
        {
            connection.sendPacket(new DisconnectPacket("Could not find this room ~Taah"));
        }


    }
}
