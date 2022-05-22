package dev.taah.handler;

import dev.taah.CrewmateServer;
import dev.taah.connection.PlayerConnection;
import dev.taah.packet.AbstractPacket;
import dev.taah.util.PacketBuffer;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufUtil;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;

import java.util.UUID;

public class PacketHandler extends SimpleChannelInboundHandler<ByteBuf>
{
    @Override
    protected void channelRead0(ChannelHandlerContext ctx, ByteBuf msg) throws Exception
    {
        PlayerConnection connection;

        if (ctx.channel().hasAttr(PlayerConnection.CONNECTION_STRING)) {
            connection = ctx.channel().attr(PlayerConnection.CONNECTION_STRING).get();
        } else {
            connection = new PlayerConnection(ctx.channel(), UUID.randomUUID());
            ctx.channel().attr(PlayerConnection.CONNECTION_STRING).set(connection);
        }
        var buffer = new PacketBuffer(msg);
        System.out.println(ByteBufUtil.prettyHexDump(buffer));
        byte tag = buffer.readByte();
        AbstractPacket packet = CrewmateServer.HANDLER.getPacket(tag);
        if (msg.readableBytes() < 1) {
            System.out.printf("Received packet from user %s with packet ID %s and length %s%n", connection.getClientName() == null ? "N/A" : connection.getClientName(), tag, buffer.readableBytes());
            return;
        }
        int nonce = buffer.readUnsignedShort();
        System.out.printf("Received packet from user %s with nonce %s, packet ID %s, and length %s%n", connection.getClientName() == null ? "N/A" : connection.getClientName(), nonce, tag, buffer.readableBytes());
        if (packet != null) {
            packet.setNonce(nonce);
            System.out.printf("Deserializing packet %s%n", packet.getClass().getSimpleName());
            packet.deserialize(buffer);
            packet.processPacket(packet, connection);
        }
//        System.out.println("Received packet with tag: " + tag);
//        var context = CrewmateServer.getOrCreate((InetSocketAddress) ctx.channel().remoteAddress(), ctx);
//        switch (tag)
//        {
//            case 0x08 -> {
//
//                var hazelVersion = buffer.readByte();
//                var clientVersion = buffer.readInt32();
//                var username = buffer.readString();
//                System.out.println("Hazel Version: " + hazelVersion);
//                System.out.println("Client Version: " + clientVersion);
//                int year = (int) floor(clientVersion / 25000);
//                int month = (int) floor((clientVersion %= 25000) / 1800);
//                int day = (int) floor((clientVersion %= 1800) / 50);
//                int revision = clientVersion % 50;
//                /*System.out.println("Received Hello Packet on version " + String.format("%s.%s.%s.%s", year, month, day, revision));
//
//                ctx.writeAndFlush(finish).sync();
//                System.out.println("Writing: " + ByteBufUtil.prettyHexDump(finish));;*/
//            }
//        }
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable e) {
        ctx.channel().close();
    }

    @Override
    public void channelActive(ChannelHandlerContext ctx) throws Exception {
        super.channelActive(ctx);
    }

    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg) {
        try {
            //super.channelRead(ctx, msg);
            ByteBuf buf = (ByteBuf) msg;
            channelRead0(ctx, buf);
        } catch (Exception e) {
            e.printStackTrace();
        }

        //channelRead0(ctx, channel.);
    }

}
