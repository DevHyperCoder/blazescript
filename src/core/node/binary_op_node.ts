import { Position } from "../../error/position.ts";
import { Token } from "../token.ts";
import { NumberNode } from "./number_nodes.ts";
import { UnaryOpNode } from "./unary_op_node.ts";

export class BinOpNode {
  public positionStart: Position;
  public positionEnd: Position;
  constructor(
    public leftNode: UnaryOpNode | NumberNode | BinOpNode,
    public opToken: Token,
    public rightNode: UnaryOpNode | NumberNode | BinOpNode,
  ) {
    this.positionStart = this.leftNode.positionStart;
    this.positionEnd = this.rightNode.positionEnd;
  }

  public represent(): string {
    return `(${this.leftNode.represent()}, ${this.opToken.represent()}, ${this.rightNode.represent()})`;
  }
}
